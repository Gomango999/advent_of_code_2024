use itertools::Itertools;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::{Add, Mul};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Vec2 {
    pub x: i64,
    pub y: i64,
}

impl Vec2 {
    pub fn new(x: i64, y: i64) -> Self {
        Vec2 { x, y }
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, oth: Self) -> Self::Output {
        Vec2::new(self.x + oth.x, self.y + oth.y)
    }
}

impl Mul<i64> for Vec2 {
    type Output = Self;

    fn mul(self, oth: i64) -> Self::Output {
        Vec2::new(self.x * oth, self.y * oth)
    }
}

pub struct Config {
    pub a_button: Vec2,
    pub b_button: Vec2,
    pub prize: Vec2,
}

pub fn parse() -> Vec<Config> {
    let file = File::open("src/solutions/p13/in.txt").unwrap();
    let reader = io::BufReader::new(file);

    let re1 = Regex::new(r"Button .: X\+(\d+), Y\+(\d+)").unwrap();
    let re2 = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    reader
        .lines()
        .chunks(4)
        .into_iter()
        .map(|lines| {
            let lines: Vec<String> = lines.collect::<Result<_, _>>().unwrap();
            let (_, [ax, ay]) = re1.captures(&lines[0]).unwrap().extract();
            let (_, [bx, by]) = re1.captures(&lines[1]).unwrap().extract();
            let (_, [px, py]) = re2.captures(&lines[2]).unwrap().extract();
            let parse_i64 = |s: &str| s.parse::<i64>().unwrap();
            let vec_from_str = |s1: &str, s2: &str| Vec2::new(parse_i64(s1), parse_i64(s2));
            let a_button = vec_from_str(ax, ay);
            let b_button = vec_from_str(bx, by);
            let prize = vec_from_str(px, py);
            Config {
                a_button,
                b_button,
                prize,
            }
        })
        .collect()
}
