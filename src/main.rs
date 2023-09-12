// This file shall be removed when the code is complete
//

use std::fs;

use bdecode::{bencode::Bencode, torrent::Torrent, Error, Result};

fn main() -> Result<()> {
    let vec_content = fs::read("./test.torrent").expect("Cannot unwrap");

    let mut content = String::new();

    for c in vec_content {
        content.push(c as char);
    }

    // let iterable = content.chars().collect::<Vec<char>>();

    let d = match Torrent::from(&content) {
        Ok(dict) => dict,
        Err(e) => {
            return Err(Error::from(format!("{e}")));
        }
    };

    // let dict =
    // Bencode::decode_dictionary(&['d', 'i', '4', '2', 'e', '4', ':', 's', 'p', 'a', 'm', 'e']);

    // println!("{dict:?}");

    Ok(())
}
