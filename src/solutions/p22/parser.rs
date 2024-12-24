use std::fs::File;
use std::io::{self, BufRead};

pub fn parse() -> Vec<u64> {
    let file = File::open("src/solutions/p22/in.txt").unwrap();
    let reader = io::BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            line.parse::<u64>().unwrap()
        })
        .collect()
}
