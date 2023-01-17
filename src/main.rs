use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
    if let Some(path) = std::env::args().skip(1).next() {
        if !path.ends_with(".aria2") {
            return;
        }
        let file = match File::open(&path) {
            Ok(file) => file,
            Err(_) => return,
        };
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer).unwrap();

        let len = i32::from_be_bytes(buffer[6..10].try_into().unwrap());
        if len > 0 {
            let hash = buffer[10..(10 + len as usize)]
                .into_iter()
                .map(|c| format!("{:02x}", c))
                .collect::<String>();
            println!("{:?}", "magnet:?xt=urn:btih:".to_owned() + &hash);
        }
    }
}