use std::fs::File;
use std::io::{self, BufRead};

enum InputState {
    Rules,
    Updates,
}

pub struct Input {
    pub rules: Vec<(i32, i32)>,
    pub manuals: Vec<Vec<i32>>,
}

pub fn parse() -> Input {
    let file = File::open("src/solutions/p05/in.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut rules = vec![];
    let mut manuals = vec![];
    let mut state = InputState::Rules;
    for line in reader.lines() {
        let line = line.unwrap();
        match state {
            InputState::Rules => {
                if line.trim().is_empty() {
                    state = InputState::Updates;
                    continue;
                }

                let rule: Vec<i32> = line.split("|").map(|x| x.parse::<i32>().unwrap()).collect();
                rules.push((rule[0], rule[1]));
            }
            InputState::Updates => {
                let manual: Vec<i32> = line.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
                manuals.push(manual)
            }
        }
    }

    Input { rules, manuals }
}
