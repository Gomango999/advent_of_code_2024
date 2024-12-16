use std::fs::File;
use std::io::{self, BufRead};

pub fn parse_file() -> Vec<Vec<char>> {
    let file = File::open("src/solutions/p04/in.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut grid = vec![];
    for line in reader.lines() {
        let row = line.unwrap().chars().collect();
        grid.push(row)
    }
    grid
}
