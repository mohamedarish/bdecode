use bdecode::decode::{decode, decode_object, decoder};

fn main() {
    // let result = decode("d3:bar4:spam3:fooi42ee".to_string());

    println!(
        // "{result}\n{}\n{}",
        "{}",
        decoder("d3:bar4:spam3:fooi42edi1e4:soogei78932ee".to_string(), 0).0
    );
}
