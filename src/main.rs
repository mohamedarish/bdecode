// This file shall be removed when the code is complete
//

use std::fs;

use bdecode::{bencode::Bencode, torrent::Torrent, Error, Result};

fn main() -> Result<()> {
    // d8:announce43:udp://tracker.coppersurfer.tk:6969/announce10:created by13:uTorrent/187013:creation datei1462355939e8:encoding5:UTF-84:infod6:lengthi124234e4:name9:puppy.jpg12:piece lengthi16384e6:pieces160:T�k�/�_(�S\u0011h%���+]q\'B\u0018�٠:����p"�j����1-g"\u0018�s(\u001b\u000f���V��=�h�m\u0017a�nF�2���N\r�ǩ�_�\u001e"2���\'�wO���-;\u0004ע\u0017�ؑ��L&����0\u001f�D_9��\t\\��O�h,n\u001a5g�(��仑,�\\߰�%��U��\u0019��C\u0007>��df��ee
    //

    let mut content = fs::read_to_string("./test.torrent").expect("Cannot unwrap");

    content.pop();

    // println!("{} {}", content, content.len());

    while let Some(pos) = content.find("\\u") {
        let mut new_fin = String::new();

        content.remove(pos);
        content.remove(pos);
        new_fin.push(content.remove(pos));

        new_fin.push(content.remove(pos));
        new_fin.push(content.remove(pos));
        new_fin.push(content.remove(pos));

        let code_point = u32::from_str_radix(&new_fin, 16).expect("Cannot unwrap");

        let s = char::from_u32(code_point).expect("Cannot unwrap");

        content.insert(pos, s);
    }

    // println!("{content} {length}", length = content.len());

    let iterable = content.chars().collect::<Vec<char>>();

    // println!("{} {:?}", iterable.len(), iterable);

    let d = match Torrent::from(&content) {
        Ok(dict) => dict,
        Err(e) => {
            return Err(Error::from(format!("{e}")));
        }
    };

    Ok(())
}
