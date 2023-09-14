use std::collections::HashMap;

use crate::{Bencode, Decode, Error, Result};

pub struct Torrent {
    pub info: Info,
    pub announce: String,
    pub announce_list: Option<Vec<Vec<String>>>,
    pub creation_date: Option<i64>,
    pub comment: Option<String>,
    pub created_by: Option<String>,
    pub encoding: Option<String>,
}

pub struct Info {
    pub name: String,
    pub length: Option<i64>,
    pub md5sum: Option<i64>,
    pub files: Option<Vec<File>>,
    pub piece_length: i64,
    pub pieces: String,
    pub private: bool,
}

pub struct File {
    pub length: i64,
    pub path: String,
    pub md5sum: Option<i64>,
}

impl Info {
    fn from(dict: &HashMap<String, Bencode>) -> Result<Self> {
        let name = match dict.get("name") {
            Some(bencode_string) => {
                match Decode::extract_string(bencode_string, false, "info -> name") {
                    Ok(string) => string,
                    Err(e) => return Err(e),
                }
            }
            None => {
                return Err(Error::RequiredFieldNotFound {
                    name: "info -> name",
                });
            }
        };

        let piece_length = match dict.get("piece length") {
            Some(bencode_int) => {
                match Decode::extract_integer(bencode_int, false, "info -> piece length") {
                    Ok(integer) => integer,
                    Err(e) => return Err(e),
                }
            }
            None => {
                return Err(Error::RequiredFieldNotFound {
                    name: "info -> piece length",
                });
            }
        };

        let pieces = match dict.get("pieces") {
            Some(bencode_string) => {
                match Decode::extract_string(bencode_string, false, "info -> pieces") {
                    Ok(string) => string,
                    Err(e) => return Err(e),
                }
            }
            None => {
                return Err(Error::RequiredFieldNotFound {
                    name: "info -> pieces",
                });
            }
        };

        let private = match dict.get("private") {
            Some(bencode_int) => {
                match Decode::extract_integer(bencode_int, true, "info -> private") {
                    Ok(integer) => Some(integer),
                    Err(e) => return Err(e),
                }
            }
            None => None,
        };

        let (length, md5sum, files) = if dict.keys().any(|key| key == "files") {
            let files = match Self::for_directory(dict) {
                Ok(files) => Some(files),
                Err(e) => return Err(e),
            };

            (None, None, files)
        } else {
            let (length, md5sum) = match Self::for_file(dict) {
                Ok(tup) => tup,
                Err(e) => return Err(e),
            };

            (Some(length), md5sum, None)
        };

        Ok(Self {
            name,
            length,
            md5sum,
            files,
            piece_length,
            pieces,
            private: private.is_some(),
        })
    }

    fn for_file(dict: &HashMap<String, Bencode>) -> Result<(i64, Option<i64>)> {
        let length = match dict.get("length") {
            Some(bencode_int) => {
                match Decode::extract_integer(bencode_int, false, "info -> length") {
                    Ok(integer) => integer,
                    Err(e) => return Err(e),
                }
            }
            None => {
                return Err(Error::RequiredFieldNotFound {
                    name: "info -> length",
                });
            }
        };

        let md5sum = match dict.get("md5sum") {
            Some(bencode_int) => match Decode::extract_integer(bencode_int, true, "info -> md5sum")
            {
                Ok(integer) => Some(integer),
                Err(e) => return Err(e),
            },
            None => None,
        };

        Ok((length, md5sum))
    }

    fn for_directory(dict: &HashMap<String, Bencode>) -> Result<Vec<File>> {
        let file_list = match dict.get("file") {
            Some(bencode_list) => match Decode::extract_list(bencode_list, false, "info -> file") {
                Ok(list) => list,
                Err(e) => return Err(e),
            },
            None => {
                return Err(Error::RequiredFieldNotFound {
                    name: "info -> file",
                });
            }
        };

        let mut files = Vec::<File>::new();

        for bencode_file in file_list {
            let file = match Decode::extract_dictionary(&bencode_file, false, "info -> file") {
                Ok(dictionary) => dictionary,
                Err(e) => return Err(e),
            };

            let final_file = match File::from(&file) {
                Ok(file) => file,
                Err(e) => return Err(e),
            };

            files.push(final_file);
        }

        Ok(files)
    }
}

impl File {
    fn from(dict: &HashMap<String, Bencode>) -> Result<Self> {
        let length = match dict.get("length") {
            Some(bencode_int) => {
                match Decode::extract_integer(bencode_int, false, "info -> file -> length") {
                    Ok(integer) => integer,
                    Err(e) => return Err(e),
                }
            }
            None => {
                return Err(Error::RequiredFieldNotFound {
                    name: "info -> file -> length",
                });
            }
        };

        let path = match dict.get("path") {
            Some(bencode_string) => {
                match Decode::extract_string(bencode_string, false, "info -> file -> path") {
                    Ok(string) => string,
                    Err(e) => return Err(e),
                }
            }
            None => {
                return Err(Error::RequiredFieldNotFound {
                    name: "info -> file -> path",
                });
            }
        };

        let md5sum = match dict.get("md5sum") {
            Some(bencode_int) => {
                match Decode::extract_integer(bencode_int, true, "info -> file -> md5sum") {
                    Ok(integer) => Some(integer),
                    Err(e) => return Err(e),
                }
            }
            None => None,
        };

        Ok(Self {
            length,
            path,
            md5sum,
        })
    }
}

impl Torrent {
    /// This function constructs the torrent struct from a string
    ///
    /// # Examples
    /// ```
    /// use bendecode::torrent::Torrent;
    ///
    /// let content = "d9:announce1:a4:infod12:piecelengthi1e6:pieces1:a4:name1:a6:lengthi1eee";
    ///     
    /// let torrent = Torrent::from(content);
    /// ````
    ///
    /// # Errors
    ///
    /// - [``RequiredFieldNotFound``](../enum.Error.html#variant.RequiredFieldNotFound)
    /// - [``Error::InvalidFieldType``](../enum.Error.html#variant.InvalidFieldType)
    pub fn from(content: &str) -> Result<Self> {
        let final_type = match Bencode::decode(content) {
            Ok(t) => t,
            Err(e) => return Err(e),
        };

        let torrent = match Decode::extract_dictionary(&final_type, true, "main_dict") {
            Ok(t) => t,
            Err(e) => return Err(e),
        };
        Self::from_dict(&torrent)
    }

    fn from_dict(dict: &HashMap<String, Bencode>) -> Result<Self> {
        let info_dict = match dict.get("info") {
            Some(bencode_dictionary) => {
                match Decode::extract_dictionary(bencode_dictionary, false, "info") {
                    Ok(dict) => dict,
                    Err(e) => return Err(e),
                }
            }
            None => {
                return Err(Error::RequiredFieldNotFound { name: "info" });
            }
        };

        let info = match Info::from(&info_dict) {
            Ok(e) => e,
            Err(e) => return Err(e),
        };

        let announce = match dict.get("announce") {
            Some(bencode_string) => {
                match Decode::extract_string(bencode_string, false, "announce") {
                    Ok(string) => string,
                    Err(e) => return Err(e),
                }
            }
            None => {
                return Err(Error::RequiredFieldNotFound { name: "announce" });
            }
        };

        let announce_list = match dict.get("announce-list") {
            Some(bencode_list) => match Decode::extract_list(bencode_list, true, "announce-list") {
                Ok(list) => match Self::find_announce_list(&list) {
                    Ok(final_list) => Some(final_list),
                    Err(e) => return Err(e),
                },
                Err(e) => return Err(e),
            },
            None => None,
        };

        let creation_date = match dict.get("creation date") {
            Some(bencode_int) => {
                match Decode::extract_integer(bencode_int, true, "creation date") {
                    Ok(int) => Some(int),
                    Err(e) => return Err(e),
                }
            }
            None => None,
        };

        let comment = match dict.get("comment") {
            Some(bencode_string) => match Decode::extract_string(bencode_string, true, "comment") {
                Ok(string) => Some(string),
                Err(e) => return Err(e),
            },
            None => None,
        };

        let created_by = match dict.get("created by") {
            Some(bencode_string) => {
                match Decode::extract_string(bencode_string, true, "created by") {
                    Ok(string) => Some(string),
                    Err(e) => return Err(e),
                }
            }
            None => None,
        };

        let encoding = match dict.get("encoding") {
            Some(bencode_string) => {
                match Decode::extract_string(bencode_string, true, "encoding") {
                    Ok(string) => Some(string),
                    Err(e) => return Err(e),
                }
            }
            None => None,
        };

        Ok(Self {
            info,
            announce,
            announce_list,
            creation_date,
            comment,
            created_by,
            encoding,
        })
    }

    fn find_announce_list(list: &Vec<Bencode>) -> Result<Vec<Vec<String>>> {
        let mut announce_list = Vec::<Vec<String>>::new();
        for element in list {
            let mut inner_announce = Vec::<String>::new();
            match Decode::extract_list(element, true, "announce_list") {
                Ok(list) => {
                    for el in list {
                        match Decode::extract_string(&el, true, "announce_list") {
                            Ok(string) => inner_announce.push(string),
                            Err(e) => return Err(e),
                        }
                    }
                }
                Err(e) => return Err(e),
            }

            announce_list.push(inner_announce);
        }

        Ok(announce_list)
    }
}
