use std::fs::File;
use std::io::{self, BufRead};

pub fn parse() -> Vec<Vec<i32>> {
    let file = File::open("src/p02/in.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut reports = vec![];
    for line in reader.lines() {
        let levels: Vec<i32> = line
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        reports.push(levels);
    }
    return reports;
}
