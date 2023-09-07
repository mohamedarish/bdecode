use bdecode::decode::{decode, decode_object};

fn main() {
    let result = decode("d3:bar4:spam3:fooi42ee".to_string());

    println!(
        "{result}\n{}",
        decode_object(Vec::from([
            'd', '3', ':', 'b', 'a', 'r', '4', ':', 's', 'p', 'a', 'm', '3', ':', 'f', 'o', 'o',
            'i', '4', '2', 'e', 'e'
        ]))
    );
}
