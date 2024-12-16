use crate::solutions::common::vec2::Vec2;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};

pub struct Robot {
    pub pos: Vec2,
    pub vel: Vec2,
}

pub fn parse() -> Vec<Robot> {
    let file = File::open("src/solutions/p14/in.txt").unwrap();
    let reader = io::BufReader::new(file);

    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (_, [px, py, vx, vy]) = re.captures(&line).unwrap().extract();
            let parse_i64 = |s: &str| s.parse::<i64>().unwrap();
            let vec_from_str = |s1: &str, s2: &str| Vec2::new(parse_i64(s1), parse_i64(s2));
            let pos = vec_from_str(px, py);
            let vel = vec_from_str(vx, vy);
            Robot { pos, vel }
        })
        .collect()
}
