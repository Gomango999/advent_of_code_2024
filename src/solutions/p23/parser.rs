use std::fs::File;
use std::io::{self, BufRead};

pub fn parse() -> Vec<(String, String)> {
    let file = File::open("src/solutions/p23/in.txt").unwrap();
    let reader = io::BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            let data: Vec<String> = line
                .unwrap()
                .split("-")
                .map(|str| str.to_string())
                .collect();
            (data[0].to_string(), data[1].to_string())
        })
        .collect()
}
