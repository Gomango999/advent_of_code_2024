use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone, Debug)]
pub struct Calibration {
    pub target: i64,
    pub nums: Vec<i64>,
}

pub fn parse() -> Vec<Calibration> {
    let file = File::open("src/solutions/p07/in.txt").unwrap();
    let reader = io::BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let data: Vec<&str> = line.split(": ").collect();
            let target = data[0].parse::<i64>().unwrap();
            let nums = data[1]
                .split(" ")
                .map(|x| x.parse::<i64>().unwrap())
                .collect();
            Calibration { target, nums }
        })
        .collect()
}
