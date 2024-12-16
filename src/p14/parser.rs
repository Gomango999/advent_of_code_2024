use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::{Add, Div, Mul, Rem};

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
impl Div<i64> for Vec2 {
    type Output = Self;

    fn div(self, oth: i64) -> Self::Output {
        Vec2::new(self.x / oth, self.y / oth)
    }
}

impl Rem for Vec2 {
    type Output = Self;
    fn rem(self, oth: Vec2) -> Self::Output {
        Vec2::new((self.x % oth.x + oth.x) % oth.x, (self.y % oth.y + oth.y) % oth.y)
    }
}

pub struct Robot {
    pub pos: Vec2,
    pub vel: Vec2,
}

pub fn parse() -> Vec<Robot> {
    let file = File::open("src/p14/in.txt").unwrap();
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
