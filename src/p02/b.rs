use std::fs::File;
use std::io::{self, BufRead};

fn parse() -> Vec<Vec<i32>> {
    let file = File::open("src/p02/in.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut reports = vec![];
    for line in reader.lines() {
        let levels: Vec<i32> = line
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        reports.push(levels);
    }
    return reports;
}

fn is_safe(report: &Vec<i32>) -> bool {
    let increasing = report.last().unwrap() > report.first().unwrap();
    let mut safe = true;
    for i in 0..(report.len() - 1) {
        if increasing && report[i] > report[i + 1] {
            safe = false;
            break;
        }
        if !increasing && report[i] < report[i + 1] {
            safe = false;
            break;
        }
        let diff = (report[i] - report[i + 1]).abs();
        if diff < 1 || diff > 3 {
            safe = false;
            break;
        }
    }
    safe
}

fn solve(reports: Vec<Vec<i32>>) {
    let mut count = 0;
    for report in reports {
        let mut safe = false;
        safe = safe | is_safe(&report);
        for i in 0..report.len() {
            let mut alternate_report = report.clone();
            alternate_report.remove(i);
            safe = safe | is_safe(&alternate_report);
        }
        if safe {
            count += 1;
        }
    }
    println!("{count}");
}

pub fn run() {
    let reports = parse();
    solve(reports);
}