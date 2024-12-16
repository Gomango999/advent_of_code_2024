use std::fs::File;
use std::io::{self, Read};

pub fn parse_file() -> String {
    let file = File::open("src/p03/in.txt").unwrap();
    let mut reader = io::BufReader::new(file);

    let mut content = String::new();
    reader.read_to_string(&mut content).unwrap();
    let content = content.replace('\n', "");
    content
}
