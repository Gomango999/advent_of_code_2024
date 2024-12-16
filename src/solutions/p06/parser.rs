use std::fs::File;
use std::io::{self, BufRead};

pub fn parse() -> Vec<Vec<char>> {
    let file = File::open("src/solutions/p06/in.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut grid = vec![];
    for line in reader.lines() {
        let row: Vec<char> = line.unwrap().chars().collect();
        grid.push(row);
    }
    grid
}
