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

fn solve(mut v1: Vec<i32>, mut v2: Vec<i32>) {
    v1.sort();
    v2.sort();
    let diff = v1
        .into_iter()
        .zip(v2.into_iter())
        .map(|(a, b)| (a - b).abs())
        .sum::<i32>();
    println!("{diff}");
}

pub fn run() {
    let (v1, v2) = parse();
    solve(v1, v2);
}
