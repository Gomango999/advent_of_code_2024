use super::parser;
use regex::Regex;

pub fn solve() {
    let text = parser::parse_file();

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
