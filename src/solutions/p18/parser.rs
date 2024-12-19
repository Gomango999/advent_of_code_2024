use std::fs::File;
use std::io::{self, BufRead};

pub fn parse() -> Vec<(usize, usize)> {
    let file = File::open("src/solutions/p18/in.txt").unwrap();
    let reader = io::BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let coords: Vec<_> = line
                .split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            (coords[0], coords[1])
        })
        .collect()
}
