use std::fs::File;
use std::io::{self, BufRead};

fn parse() -> () {
    let file = File::open("src/p00/in.txt").unwrap();
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
