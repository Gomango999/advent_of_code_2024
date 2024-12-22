use std::fs::File;
use std::io::{self, BufRead};

pub type Code = Vec<char>;

pub fn parse() -> Vec<Code> {
    let file = File::open("src/solutions/p21/in.txt").unwrap();
    let reader = io::BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            line.chars().collect()
        })
        .collect()
}
