use std::fs::File;
use std::io::{self, BufRead};

fn parse() -> () {
    let file = File::open("").unwrap();
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        todo!();
    }
    todo!();
}

fn solve() {
    todo!()
}

pub fn run() {
    let () = parse();
    solve();
}
