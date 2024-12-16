use std::fs::File;
use std::io::{self, BufRead};

pub fn parse() -> Vec<Vec<char>> {
    let file = File::open("src/solutions/p12/in.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut garden: Vec<Vec<char>> = vec![];
    for line in reader.lines() {
        let row = line.unwrap().chars().collect();
        garden.push(row)
    }
    garden
}
