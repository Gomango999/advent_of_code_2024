use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

pub struct Input {
    pub height: i32,
    pub width: i32,
    pub antennae: HashMap<char, Vec<(i32, i32)>>,
}

pub fn parse() -> Input {
    let file = File::open("src/p08/in.txt").unwrap();
    let reader = io::BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    let height = lines.len() as i32;
    let width = lines[0].len() as i32;

    let mut antennae: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for y in 0..height {
        for x in 0..width {
            let char = lines[y as usize].chars().nth(x as usize).unwrap();
            if char != '.' {
                antennae
                    .entry(char)
                    .or_insert_with(Vec::new)
                    .push((x as i32, y as i32));
            }
        }
    }

    Input {
        height,
        width,
        antennae,
    }
}
