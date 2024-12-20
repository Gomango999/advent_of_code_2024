use super::parser;
use regex::Regex;

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Do,
    Dont,
    Mul(i32, i32),
}

#[derive(Debug, Copy, Clone)]
struct Match {
    pos: usize,
    instruction: Instruction,
}

pub fn solve() {
    let text = parser::parse_file();

    let re1 = Regex::new(r"do\(\)").unwrap();
    let re2 = Regex::new(r"don't\(\)").unwrap();
    let re3 = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    // collect and sort all occurences of instructions in order of position in the text.
    let do_matches: Vec<Match> = re1
        .find_iter(&text)
        .map(|m| Match {
            pos: m.start(),
            instruction: Instruction::Do,
        })
        .collect();
    let dont_matches: Vec<Match> = re2
        .find_iter(&text)
        .map(|m| Match {
            pos: m.start(),
            instruction: Instruction::Dont,
        })
        .collect();
    let mul_matches: Vec<Match> = re3
        .captures_iter(&text)
        .map(|caps| {
            let m = caps.get(0).unwrap();
            let (_, [num1, num2]) = caps.extract();
            let num1 = num1.parse::<i32>().unwrap();
            let num2 = num2.parse::<i32>().unwrap();
            Match {
                pos: m.start(),
                instruction: Instruction::Mul(num1, num2),
            }
        })
        .collect();
    let mut matches = [do_matches, dont_matches, mul_matches].concat();
    matches.sort_by(|m1, m2| m1.pos.cmp(&m2.pos));

    // simulate commands
    let mut total = 0;
    let mut enabled = true;
    for m in matches {
        match m.instruction {
            Instruction::Do => enabled = true,
            Instruction::Dont => enabled = false,
            Instruction::Mul(num1, num2) => {
                if enabled {
                    total += num1 * num2
                }
            }
        }
    }
    println!("{total}")
}
