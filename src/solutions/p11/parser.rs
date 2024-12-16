use std::fs::File;
use std::io::{self, BufRead};

pub fn parse() -> Vec<u64> {
    let file = File::open("src/p11/in.txt").unwrap();
    let mut reader = io::BufReader::new(file);

    let mut line = String::new();
    let _ = reader.read_line(&mut line);
    let nums = line.split(" ").map(|x| x.parse::<u64>().unwrap()).collect();
    nums
}
