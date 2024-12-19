use std::fs::File;
use std::io::{self, BufRead};

pub type Pattern = Vec<char>;

pub struct Input {
    pub towels: Vec<Pattern>,
    pub designs: Vec<Pattern>,
}

pub fn parse() -> Input {
    let file = File::open("src/solutions/p19/in.txt").unwrap();
    let mut reader = io::BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let towels: Vec<Vec<char>> = line
        .trim()
        .split(", ")
        .map(|towel| towel.chars().collect())
        .collect();

    let mut designs = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.trim();
        if line != "" {
            let design: Vec<char> = line.chars().collect();
            designs.push(design)
        }
    }
    Input { towels, designs }
}
