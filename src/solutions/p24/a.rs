use super::parser::*;

pub fn solve() {
    let mut machine = parse();

    let output = machine.run();

    println!("{}", output);
}
