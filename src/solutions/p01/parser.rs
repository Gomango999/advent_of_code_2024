use std::fs::File;
use std::io::{self, BufRead};

pub fn parse_file() -> (Vec<i32>, Vec<i32>) {
    let file = File::open("src/p01/in.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut v1 = vec![];
    let mut v2 = vec![];
    for line in reader.lines() {
        let nums: Vec<i32> = line
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        v1.push(nums[0]);
        v2.push(nums[1]);
    }
    return (v1, v2);
}
