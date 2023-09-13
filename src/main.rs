// This file shall be removed when the code is complete
//

use std::fs;

use bdecode::{
    bencode::{Bencoder, Bencoding},
    Result,
};

fn main() -> Result<()> {
    let vec_content = fs::read("./test.torrent").expect("Cannot unwrap");

    let mut content = String::new();

    for c in vec_content {
        content.push(c as char);
    }

    let d = Bencoder::decode(content.as_str()).unwrap();

    match d {
        Bencoding::Int(int) => {
            println!("{int}");
        }
        Bencoding::Str(string) => {
            println!("{string}");
        }
        Bencoding::Dict(dict) => {
            for (key, value) in &*dict {
                print!("{key}: ");

                match value {
                    Bencoding::List(list) => {
                        for element in &**list {
                            println!("{element:?}");
                        }
                    }
                    Bencoding::Dict(dict) => {
                        for (k, v) in &**dict {
                            print!("{k}: ");
                            if k != "pieces" {
                                println!("{v:?}");
                            } else {
                                println!();
                            }
                        }
                    }
                    Bencoding::Str(string) => {
                        println!("{string}");
                    }
                    Bencoding::Int(int) => {
                        println!("{int}");
                    }
                }
            }
        }
        Bencoding::List(list) => {
            for element in &*list {
                println!("{element:?}");
            }
        }
    }

    // let iterable = content.chars().collect::<Vec<char>>();

    // let d = match Torrent::from(&content) {
    //     Ok(dict) => dict,
    //     Err(e) => {
    //         return Err(Error::from(format!("{e}")));
    //     }
    // };

    // let dict =
    // Bencode::decode_dictionary(&['d', 'i', '4', '2', 'e', '4', ':', 's', 'p', 'a', 'm', 'e']);

    // println!("{dict:?}");

    Ok(())
}
