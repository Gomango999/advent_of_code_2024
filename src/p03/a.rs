use regex::Regex;
use std::fs::File;
use std::io::{self, Read};

fn parse() -> String {
    let file = File::open("src/p03/in.txt").unwrap();
    let mut reader = io::BufReader::new(file);

    let mut content = String::new();
    reader.read_to_string(&mut content).unwrap();
    let content = content.replace('\n', "");
    content
}

fn solve(text: String) {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut total = 0;
    let result: i32 = re
        .captures_iter(&text)
        .map(|caps| {
            let (_, [num1, num2]) = caps.extract();
            let num1 = num1.parse::<i32>().unwrap();
            let num2 = num2.parse::<i32>().unwrap();
            num1 * num2
        })
        .sum();
    total += result;
    println!("{total}")
}

pub fn run() {
    let text = parse();
    solve(text);
}
