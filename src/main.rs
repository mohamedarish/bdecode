// This file shall be removed when the code is complete
//

use bdecode::{bencode::Bencode, decoder::Torrent};

fn main() {
    // let d = Torrent::from("d3:bar4:spam3:fooi42ee");

    let p = Bencode::decode_dictionary(&[
        'd', '3', ':', 'b', 'a', 'r', '4', ':', 's', 'p', 'a', 'm', '3', ':', 'f', 'o', 'o', 'i',
        '4', '2', 'e', 'e',
    ]);

    println!("{:?}", p);
}
