use std::fs::File;
use std::io::{self, BufRead};

fn parse() -> (Vec<i32>, Vec<i32>) {
    let file = File::open("src/p01/in1.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut v1 = vec![];
    let mut v2 = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        v1.push(nums[0]);
        v2.push(nums[1]);
    }
    return (v1, v2);
}

fn solve(v1: Vec<i32>, v2: Vec<i32>) {
    let mut total = 0;
    for &a in v1.iter() {
        let mut count = 0;
        for &b in v2.iter() {
            if a == b {
                count += 1;
            }
        }
        total += a * count;
    }
    println!("{total}");
}

pub fn run() {
    let (v1, v2) = parse();
    solve(v1, v2);
}
