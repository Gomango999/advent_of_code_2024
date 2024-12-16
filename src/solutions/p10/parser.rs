use std::fs::File;
use std::io::{self, BufRead};

pub fn parse() -> Vec<Vec<u32>> {
    let file = File::open("src/p10/in.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut map = vec![];
    for line in reader.lines() {
        let row = line
            .unwrap()
            .chars()
            .map(|x| x.to_digit(10).unwrap())
            .collect();
        map.push(row)
    }
    map
}
